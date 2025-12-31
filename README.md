# Agile KanBan Card Game

## Summary

Simulate working teams by trying to complete a project in a limited amount of time. This allows teams to experiment with different working rules that will change how quickly they are able to complete tasks and the project.

## How the Game Is Played

Board is a KanBan board with the following columns

- Ready
- Design
- Construction
- Feedback
- Validation
- Done
  - Deployed
    - Internal Improvements
    - Business Initiatives
    - Change Requests
    - Discovery
  - Canceled

setup:

- Set up the board with 40 issues in Ready
- Deck of playing cards (shuffled)
- 3 or more players (equal to the team sizes that we are working with)

First time playing:

Each turn:

- Next up pulls a card
- If the card is black
  - Congrats, it's a good productive day. You get to do one of the following:
    - Move a card forward if you have one
      - If the card is in the far right column it can be completed
      - We can have up to 4 cards each in the deployed subsections.
      - We can have unlimited cards in canceled
      - We can have unlimited cards in deployed
    - Take ownership of a card and move it into design
    - Unblock one of your blocked cards
- Else if the card is red
  - Oh no, the day isn't that productive. You have to do all the following:
    - Block a card that isn't blocked
    - Take ownership of a new card and begin work on it

Scoring:

1 point for every card in deployed 1 point for every card in canceled 10 point bonus for each subsection in deployed that is completely filled

Subsequent playthroughs:

Set up some rules for the game. The team gets to choose what options to set. The options are:

- Maximum amount of issues in a column
- Maximum amount of issues in progress
- On a black card:
  - Can unblock another players blocked card
  - Can move another players card one column
- On a red card:
  - Instead of pulling a new card, may choose to unblock or move another players card forward

## Stories

- [x] As a player, I want to launch the game
  - [x] Frontend
    - [x] Hello world set up
    - [x] Splash screen loads with game title
- [ ] As a coordinator, I want to host a game
  - [ ] Frontend
    - [x] Form field to enter name
    - [x] Button/link to host a game
    - [x] When button is clicked the frontend sends an event to start the game
    - [x] After starting the game, Can see a lobby
  - [ ] Backend
    - [x] Hello world api set up
    - [ ] Database configured
    - [ ] route for creating a game
    - [ ] Generate game code and put into database
    - [ ] Return game code to frontend
