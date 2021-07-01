# Gomonku
A game of Gomoku integrating an AI player capable of defeating a human player as quickly as possible. To do this, we implemented a negamax with alpha prunning algorithm but also (and above all), do research and tests to find the most suitable heuristics.
# The game
Gomoku is a strategy board game traditionally played on a Go board with stones of two
different colors.

Two players take turns placing stones of their color on an intersection of the board,
and the game ends when one player manages to align five stones. Sometimes, only an
alignment of 5 can win, and sometimes 5 or more is okay. In the context of this projet,
we will consider 5 or more to be a win.

There are different interpretations on what the board size for Gomoku should be. In
the context of this project, Gomoku will be played on a 19x19 Goban, without limit to
the number of stones.

There are a great many additional rules to Gomoku aimed at making the
game more fair (regular Gomoku is proven to be unfair, a perfect first player wins 100%
of the time) and more interesting.

In the context of this project, you will play with the following additional rules :

• Capture (As in the Ninuki-renju or Pente variants) : You can remove a pair of your
opponent’s stones from the board by flanking them with your own stones (See the
appendix). This rule adds a win condition : If you manage to capture ten of your
opponent’s stones, you win the game.

• Game-ending capture : A player that manages to align five stones only wins if the
opponent can not break this alignment by capturing a pair, or if he has already lost
four pairs and the opponent can capture one more, therefore winning by capture.
There is no need for the game to go on if there is no possibility of this happening.

• No double-threes : It is forbidden to play a move that introduces two free-three
alignments, which would guarantee a win by alignment (See the appendix).
