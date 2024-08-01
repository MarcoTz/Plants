use datetime::LocalDate;

pub struct GraveyardPlant {
    name: String,
    species: String,
    planted: LocalDate,
    died: LocalDate,
    reason: String,
}
