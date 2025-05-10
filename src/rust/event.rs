#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EventType {
    Ao5(Ao5Event),
    Mo3(Mo3Event),
    Bo3(Bo3Event),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Ao5Event {
    S222,
    S333,
    S444,
    S555,
    S333oh,
    Megaminx,
    Pyraminx,
    Clock,
    Skewb,
    SQ1,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mo3Event {
    S666,
    S777,
    F333,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Bo3Event {
    B333,
    B444,
    B555,
}

impl EventType {
    pub fn from_event_id(event_id: &str) -> Option<Self> {
        match event_id {
            // Average of 5 events
            "222" => Some(Self::Ao5(Ao5Event::S222)),
            "333" => Some(Self::Ao5(Ao5Event::S333)),
            "444" => Some(Self::Ao5(Ao5Event::S444)),
            "555" => Some(Self::Ao5(Ao5Event::S555)),
            "333oh" => Some(Self::Ao5(Ao5Event::S333oh)),
            "minx" => Some(Self::Ao5(Ao5Event::Megaminx)),
            "pyram" => Some(Self::Ao5(Ao5Event::Pyraminx)),
            "clock" => Some(Self::Ao5(Ao5Event::Clock)),
            "skewb" => Some(Self::Ao5(Ao5Event::Skewb)),
            "sq1" => Some(Self::Ao5(Ao5Event::SQ1)),

            // Mean of 3 events
            "666" => Some(Self::Mo3(Mo3Event::S666)),
            "777" => Some(Self::Mo3(Mo3Event::S777)),
            "333fm" => Some(Self::Mo3(Mo3Event::F333)),

            // Best of 3 events
            "333bf" => Some(Self::Bo3(Bo3Event::B333)),
            "444bf" => Some(Self::Bo3(Bo3Event::B444)),
            "555bf" => Some(Self::Bo3(Bo3Event::B555)),

            _ => None,
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            // Average of 5 events
            Self::Ao5(Ao5Event::S222) => "222",
            Self::Ao5(Ao5Event::S333) => "333",
            Self::Ao5(Ao5Event::S444) => "444",
            Self::Ao5(Ao5Event::S555) => "555",
            Self::Ao5(Ao5Event::S333oh) => "333oh",
            Self::Ao5(Ao5Event::Megaminx) => "minx",
            Self::Ao5(Ao5Event::Pyraminx) => "pyram",
            Self::Ao5(Ao5Event::Clock) => "clock",
            Self::Ao5(Ao5Event::Skewb) => "skewb",
            Self::Ao5(Ao5Event::SQ1) => "sq1",

            // Mean of 3 events
            Self::Mo3(Mo3Event::S666) => "666",
            Self::Mo3(Mo3Event::S777) => "777",
            Self::Mo3(Mo3Event::F333) => "333fm",

            // Best of 3 events
            Self::Bo3(Bo3Event::B333) => "333bf",
            Self::Bo3(Bo3Event::B444) => "444bf",
            Self::Bo3(Bo3Event::B555) => "555bf",
        }
    }

    pub fn num_attempts(&self) -> usize {
        match self {
            Self::Ao5(_) => 5,
            Self::Mo3(_) => 3,
            Self::Bo3(_) => 3,
        }
    }
}
