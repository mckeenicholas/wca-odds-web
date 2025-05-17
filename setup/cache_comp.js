import fs from "fs";
import path from "path";
import process from "process";

const jsonFilePath = process.argv[2];

if (!jsonFilePath) {
  console.error(
    "Please provide a path to a JSON file containing competition IDs",
  );
  console.error("Usage: node cache_comp.js PATH_TO_JSON_FILE");
  process.exit(1);
}

async function main() {
  try {
    if (!fs.existsSync(jsonFilePath)) {
      console.error(`File not found: ${jsonFilePath}`);
      process.exit(1);
    }

    const fileContent = fs.readFileSync(jsonFilePath, "utf8");
    let competitionIds;

    try {
      competitionIds = JSON.parse(fileContent);
    } catch (parseError) {
      console.error(`Error parsing JSON file: ${parseError.message}`);
      process.exit(1);
    }

    if (!Array.isArray(competitionIds)) {
      console.error("The JSON file must contain an array of competition IDs");
      process.exit(1);
    }

    const outputDir = path.resolve("./public/wcif");
    if (!fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }

    console.log(`Found ${competitionIds.length} competitions to cache`);

    await Promise.allSettled(
      competitionIds.map(async (compId) => {
        try {
          console.log(`Fetching competition data for: ${compId}`);
          const compData = await fetchCompetition(compId);

          const outputPath = path.join(outputDir, `${compId}.json`);
          fs.writeFileSync(outputPath, JSON.stringify(compData));

          console.log(`Competition data saved for: ${compId}`);
        } catch (error) {
          console.error(`Error fetching ${compId}:`, error.message);
        }
      }),
    );
  } catch (error) {
    console.error("Error:", error);
    process.exit(1);
  }
}

async function fetchCompetition(id) {
  const wcaURL = `https://api.worldcubeassociation.org/competitions/${id}/wcif/public`;

  console.log(`Fetching from: ${wcaURL}`);
  const result = await fetch(wcaURL);

  if (!result.ok) {
    throw new Error(
      `Failed to fetch competition data: ${result.status} ${result.statusText}`,
    );
  }

  const fullData = await result.json();

  // Only keep required attributes
  const filteredData = {
    name: fullData.name,
    id: fullData.id,
    events: fullData.events.map((event) => ({
      id: event.id,
    })),
    schedule: {
      startDate: fullData.schedule.startDate,
    },
    persons: fullData.persons.map((person) => ({
      name: person.name,
      wcaId: person.wcaId,
      countryIso2: person.countryIso2,
      personalBests: person.personalBests.map((pb) => ({
        eventId: pb.eventId,
        best: pb.best,
        worldRanking: pb.worldRanking,
      })),
      registration: {
        wcaRegistrationId: person.registration?.wcaRegistrationId,
        eventIds: person.registration?.eventIds,
        status: person.registration?.status,
        isCompeting: person.registration?.isCompeting,
      },
    })),
  };

  return filteredData;
}

main();
