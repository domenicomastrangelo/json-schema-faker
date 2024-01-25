use rand::Rng;

const JOB_TITLES: [&str; 114] = [
    "Accountant",
    "Actor",
    "Architect",
    "Astronomer",
    "Author",
    "Bartender",
    "Biologist",
    "Blacksmith",
    "Bricklayer",
    "Bus Driver",
    "Butcher",
    "Carpenter",
    "Chef",
    "Chemist",
    "Civil Engineer",
    "Cleaner",
    "Clerk",
    "Coach",
    "Computer Programmer",
    "Concierge",
    "Construction Worker",
    "Cook",
    "Counselor",
    "Dancer",
    "Dentist",
    "Designer",
    "Detective",
    "Dietitian",
    "Director",
    "Doctor",
    "Economist",
    "Editor",
    "Electrician",
    "Engineer",
    "Farmer",
    "Fashion Designer",
    "Film Director",
    "Financial Advisor",
    "Firefighter",
    "Fisherman",
    "Florist",
    "Gardener",
    "Geologist",
    "Graphic Designer",
    "Hairdresser",
    "Historian",
    "Hotel Manager",
    "Interpreter",
    "Investigator",
    "Jeweler",
    "Journalist",
    "Judge",
    "Lawyer",
    "Lecturer",
    "Librarian",
    "Lifeguard",
    "Locksmith",
    "Magician",
    "Mechanic",
    "Meteorologist",
    "Pilot",
    "Plumber",
    "Police Officer",
    "Politician",
    "Psychologist",
    "Receptionist",
    "Reporter",
    "Researcher",
    "Retail Worker",
    "Sailor",
    "Salesperson",
    "Scientist",
    "Secretary",
    "Security Guard",
    "Singer",
    "Social Worker",
    "Software Developer",
    "Surgeon",
    "Surveyor",
    "Tailor",
    "Teacher",
    "Technician",
    "Therapist",
    "Translator",
    "Travel Agent",
    "Veterinarian",
    "Videographer",
    "Waiter",
    "Web Developer",
    "Welder",
    "Writer",
    "Zoologist",
    "Acupuncturist",
    "Animator",
    "Archaeologist",
    "Art Director",
    "Audiologist",
    "Bartender",
    "Chiropractor",
    "Composer",
    "Curator",
    "Dermatologist",
    "Ecologist",
    "Entomologist",
    "Ethnographer",
    "Genealogist",
    "Immunologist",
    "Microbiologist",
    "Neurologist",
    "Oncologist",
    "Orthodontist",
    "Pediatrician",
    "Radiologist",
    "Sociologist",
];

pub fn generate() -> serde_json::Value {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..JOB_TITLES.len());

    serde_json::json!(JOB_TITLES[index])
}
