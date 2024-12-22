CREATE TABLE plants (name string unique, species string, location string, origin string, obtained string, auto_water bool, notes string);
CREATE TABLE activities (name string, date string, plant string not null, note string, CONSTRAINT unq UNIQUE (name,date));
CREATE TABLE graveyard (name string, species string, planted string, died string, reason string);
CREATE TABLE growth (plant string, date string, height_cm real ,width_cm real, note string, health int, CONSTRAINT unq UNIQUE (plant,date));
CREATE TABLE locations (name string unique, outside bool);
CREATE TABLE species (name string unique, scientific_name string, genus string, family string, sunlight string, temp_min real, temp_max real, temp_min_opt real, temp_max_opt real, planting_distance real, ph_min real, ph_max real, watering_notes string, fertilizing_notes string, avg_watering_days int, avg_fertilizing_days int, pruning_notes string,companions string, additional_notes string);
