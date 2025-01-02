import os
import time
import requests

from dotenv import load_dotenv
load_dotenv()

# Riot API key (replace with your actual API key)
RIOT_API_KEY = os.getenv('RIOT_API_KEY')
BASE_URL = "https://<REGION>.api.riotgames.com"
HEADERS = {"X-Riot-Token": RIOT_API_KEY}

# Get Challenger tier summoners
def get_challenger_summoners(region):
    url = f"{BASE_URL}/lol/league/v4/challengerleagues/by-queue/RANKED_SOLO_5x5"
    response = requests.get(url.replace("<REGION>", region), headers=HEADERS)
    response.raise_for_status()
    data = response.json()
    return [entry['summonerId'] for entry in data['entries']]

# Get PUUID for summoners
def get_summoner_puuid(region, summoner_id):
    url = f"{BASE_URL}/lol/summoner/v4/summoners/{summoner_id}"
    response = requests.get(url.replace("<REGION>", region), headers=HEADERS)
    response.raise_for_status()
    return response.json()['puuid']

# Get match history by PUUID
def get_match_history(region, puuid, count=20):
    url = f"https://<REGION>.api.riotgames.com/lol/match/v5/matches/by-puuid/{puuid}/ids?start=0&count={count}"
    response = requests.get(url.replace("<REGION>", region), headers=HEADERS)
    response.raise_for_status()
    return response.json()

# Get match details
def get_match_details(region, match_id):
    url = f"https://<REGION>.api.riotgames.com/lol/match/v5/matches/{match_id}"
    response = requests.get(url.replace("<REGION>", region), headers=HEADERS)
    response.raise_for_status()
    return response.json()

# Filter matches by role and matchup
def filter_matches(matches, role, champ1, champ2):
    filtered_matches = []
    for match in matches:
        participants = match['info']['participants']
        for player in participants:
            if player['individualPosition'] == role and player['championName'] == champ1:
                opponent = next((p for p in participants if p['teamId'] != player['teamId'] and p['individualPosition'] == role), None)
                if opponent and opponent['championName'] == champ2:
                    filtered_matches.append({
                        "match_id": match['metadata']['matchId'],
                        "player": player,
                        "opponent": opponent
                    })
    return filtered_matches

# Main script flow
def main():
    region_routing = "asia"  # Change as needed (e.g., asia, europe, americas, sea)
    region = "kr"  # Change as needed (e.g., kr, euw1, euw2, na1...)
    role = "TOP"  # Role to filter
    champ1 = "Riven"  # Your champion
    champ2 = "Renekton"  # Opponent champion

    try:
        print("Fetching Challenger summoners...")
        summoner_ids = get_challenger_summoners(region)[:1]  # NOTE: Limit for testing
        print(f'Summoners IDS: {summoner_ids}')

        print("Fetching match histories...")
        all_matches = []
        for summoner_id in summoner_ids:
            puuid = get_summoner_details(region, summoner_id)
            print(f'Looking for player: {puuid}')
            match_ids = get_match_history(region_routing, puuid, 1)
            print(f'Recent {puuid} matches: {match_ids}')
            for match_id in match_ids:
                match_details = get_match_details(region_routing, match_id)
                all_matches.append(match_details)
                time.sleep(1.2)  # Rate limit handling

        print("Filtering matches...")
        filtered_matches = filter_matches(all_matches, role, champ1, champ2)
        print(f"Found {len(filtered_matches)} matches for {champ1} vs {champ2} in {role}.")

        for match in filtered_matches:
            print(f"Match ID: {match['match_id']}, Player: {match['player']['summonerName']}, Opponent: {match['opponent']['summonerName']}")

    except requests.exceptions.RequestException as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()

