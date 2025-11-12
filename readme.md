# TUI Minesweeper
Terminal based minesweeper built using ratatui

# How To Play
- Clone the repo and run `cargo run` in the same directory as src/
- Navigate the menu using the `up` and `down` arrows, changing the grid height and width, as well as the number of mines using the `left` and `right` arrows. The grid sized is constrained by the size of the terminal
- To start, hover over the `Start` button and press the `enter` or `space` key
- Move around the grid using `up`, `down`, `left`, and `right`. The curently selected tile is where the background cross overlaps
- To reveal a cell, press `enter` or `space`
- To flag a cell, press `f`. This is not required, and is just used to remind you where you think a mine is
- The game will end once all the empty cells are revealed