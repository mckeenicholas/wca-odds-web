const getYearsFromDate = (startDate) => {
  let today = new Date();

  let years = [];
  for (
    let year = startDate.getFullYear();
    year <= today.getFullYear();
    year++
  ) {
    years.push(year);
  }

  return years;
};

const APIRequest = async (url) => {
  try {
    const response = await fetch(url);
    if (response.ok) {
      const data = await response.json();
      return data;
    } else {
      console.error("Error fetching data:", response.statusText);
      return null;
    }
  } catch (error) {
    console.error("Error fetching data:", error);
    return null;
  }
};

const getinfo = async (persons, startDate) => {
  const years = getYearsFromDate(startDate);

  const response = await Promise.all(
    persons.map(
      async (person) =>
        await APIRequest(
          `https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/persons/${person}.json`,
        ),
    ),
  );
  const competitions = (
    await Promise.all(
      years.map(async (year) => {
        const response = await APIRequest(
          `https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/competitions/${year}.json`,
        );
        return response.items.map((comp) => {
          return { id: comp.id, date: comp.date.from };
        });
      }),
    )
  ).flat();

  console.time("Runtime");

  var compDate = {};

  for (const comp of competitions) {
    compDate[comp.id] = comp.date;
  }

  const wcaEvent = "777";
  const numSolves = 3;

  for (const person of response) {
    const results = person.results;
    const dates = Object.entries(results)
      .filter(
        ([key, values]) =>
          new Date(compDate[key]) > startDate && values[wcaEvent],
      )
      .flatMap(([_, values]) =>
        values[wcaEvent]?.flatMap((round) =>
          round.solves.slice(0, numSolves).filter((solve) => solve !== 0),
        ),
      ); // Map to return the values
    console.log(dates);
  }

  console.timeEnd("Runtime");
};

const persons = [
  "2015MCKE02",
  "2015COHE02",
  "2012PARK03",
  "2007STRO01",
  "2016KOLA02",
];
const months = 12;

let startDate = new Date();
startDate.setMonth(new Date().getMonth() - months);

getinfo(persons, startDate);
