import {
  FetchRoundResultsGraphQLResponse,
  SupportedWCAEvent,
  WCALiveCompetitionData,
  WCALiveCompetitionError,
} from "./types";

const WCA_LIVE_ENDPOINT = "https://live.worldcubeassociation.org/api";

const fetchWCALiveResults = async (
  competitionId: string,
  event: SupportedWCAEvent,
  ids: string[],
): Promise<number[][]> => {
  const wcaLiveId = await getWCALiveID(competitionId);
  const competitionData = await fetchCompetitionRounds(wcaLiveId);

  const targetEvent = competitionData.competitionEvents.find(
    (competitionEvent) => competitionEvent.event.id === event,
  );

  if (!targetEvent?.rounds || targetEvent.rounds.length === 0) {
    throw Error(
      `Event '${event}' not found or has no rounds in competition '${competitionId}'.`,
    );
  }

  const finalRound = targetEvent.rounds.reduce((acc, round) => {
    return round.number > acc.number ? round : acc;
  }, targetEvent.rounds[0]);

  const roundResultsResponse = (await fetchRoundResults(
    finalRound.id,
  )) as FetchRoundResultsGraphQLResponse;
  const processedOutput: number[][] = [];

  if (roundResultsResponse.errors && roundResultsResponse.errors.length > 0) {
    const errorMessages = roundResultsResponse.errors
      .map((e) => e.message)
      .join("; ");
    throw Error(
      `GraphQL query for round '${finalRound.id}' failed: ${errorMessages}`,
    );
  }

  if (!roundResultsResponse.data?.round?.format) {
    throw Error(
      `Essential data (round or format) missing in WCALive response for round '${finalRound.id}'.`,
    );
  }

  const roundDetails = roundResultsResponse.data.round;
  const numberOfAttempts = roundDetails.format.numberOfAttempts;
  const actualRoundResultsList = roundDetails.results || [];

  const personAttemptsMap = new Map<string, number[]>();
  for (const resultItem of actualRoundResultsList) {
    if (resultItem.person?.wcaId && resultItem.attempts) {
      const attemptValues = resultItem.attempts.map((att) => att.result);
      personAttemptsMap.set(resultItem.person.wcaId, attemptValues);
    }
  }

  for (const wcaId of ids) {
    const individualPersonAttempts: number[] = [];
    const recordedAttempts = personAttemptsMap.get(wcaId);

    for (let i = 0; i < numberOfAttempts; i++) {
      if (recordedAttempts && i < recordedAttempts.length) {
        individualPersonAttempts.push(recordedAttempts[i]);
      } else {
        individualPersonAttempts.push(0);
      }
    }
    processedOutput.push(individualPersonAttempts);
  }
  return processedOutput;
};

const fetchRoundResults = async (roundId: string) => {
  const query = `query Round($id: ID!) {
          round(id: $id) {
              format {
                  numberOfAttempts
              }
              results {
                  ...roundResult
              }
          }
      }
    
      fragment roundResult on Result {
          attempts {
              result
          }
          person {
              wcaId
          }
      }`;

  const response = await fetchGraphQL("Round", query, { id: roundId });
  return response;
};

const fetchCompetitionRounds = async (id: string) => {
  const query = `
    query Competition($id: ID!) {
          competition(id: $id) {
              competitionEvents {
                  event {
                      id
                  }
                  rounds {
                      id
                      number
                  }
              }
          }
      }
  `;

  try {
    const response = (await fetchGraphQL("Competition", query, {
      id,
    })) as WCALiveCompetitionData;

    if ("data" in response && response.data?.competition) {
      return response.data.competition;
    } else if ("errors" in response && response.errors) {
      const errorDetail =
        (response as WCALiveCompetitionError).errors.detail ||
        "Unknown GraphQL error";
      throw Error(`GraphQL query 'Competition' failed: ${errorDetail}`);
    } else {
      throw Error(
        "GraphQL query 'Competition' returned an unexpected response structure.",
      );
    }
  } catch (err) {
    if (err instanceof Error) {
      throw Error(`Fetch competition rounds error: ${err.message}`);
    }
    throw Error(
      `An unknown error occurred while fetching competition rounds: ${String(err)}`,
    );
  }
};

const getWCALiveID = async (competitionId: string) => {
  const linkUrl = `https://live.worldcubeassociation.org/link/competitions/${competitionId}`;
  const response = await fetch(linkUrl, {
    method: "GET",
    redirect: "follow",
  });

  if (!response.ok) {
    throw new Error(
      `Failed to fetch WCA Live link for ${competitionId} from ${linkUrl}: ${response.status} ${response.statusText}`,
    );
  }

  const match = response.url.match(/\/competitions\/([^/]+)/);

  if (!match || !match[1]) {
    throw Error(
      `Unable to extract WCA Live ID from redirected URL: ${response.url} (original competition ID: ${competitionId})`,
    );
  }

  const wcaLiveID = match[1];
  return wcaLiveID;
};

const fetchGraphQL = async (
  operationName: string,
  query: string,
  variables: object,
) => {
  const requestOptions = {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ operationName, variables, query }),
  };

  const response = await fetch(WCA_LIVE_ENDPOINT, requestOptions);
  if (!response.ok) {
    const errorBody = await response.text();
    throw new Error(
      `GraphQL request failed with status ${response.status} ${response.statusText}: ${errorBody}`,
    );
  }
  return await response.json();
};

export default fetchWCALiveResults;
