import fs from "fs";
import path from "path";
import process from "process";

const compId = process.argv[2];

if (!compId) {
  console.error("Please provide a competition ID as a command line argument");
  console.error("Usage: node cache_comp.js COMPETITION_ID");
  process.exit(1);
}

async function main() {
  try {
    console.log(`Fetching competition data for: ${compId}`);
    const compData = await fetchCompetition(compId);

    const outputDir = path.resolve("./public/wcif");
    if (!fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }

    const outputPath = path.join(outputDir, `${compId}.json`);
    fs.writeFileSync(outputPath, JSON.stringify(compData));

    console.log(`Competition data saved to: ${outputPath}`);
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
    events: fullData.events.map((event) => ({
      id: event.id,
    })),
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
