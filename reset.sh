#!/bin/bash
bot_config="./telegram_bot/conf.json"

rm -r Plants/*
rm -r Logs/*
echo "Remove plant species? (y/n)"
read remove_species
if [ "$remove_species" = "y" ]; then 
  rm -r PlantSpecies/*
fi 

echo "Please enter API key"
read api_key
echo "Please enter telegram user id"
read user_id
echo "{" > "$bot_config"
echo "\"api_key\" : \"$api_key\"," >> "$bot_config"
echo "\"white_list\" : [$user_id]" >> "$bot_config"
echo "}" >> "$bot_config"
