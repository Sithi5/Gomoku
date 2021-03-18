import subprocess


# Trying to import the gomoku_rust lib, if not compiled, execute the script to compile it.
try:
    import gomoku_rust
except ImportError:
    # Build rust lib
    process = subprocess.Popen("rust_compilation.sh", shell=True, stdout=subprocess.PIPE)
    process.wait()
    print(process.returncode)
    import gomoku_rust

from player import Player


class GoRules:
    table = []
    player_list = []

    def __init__(self):
        m = 19
        n = 19
        self.table = [[0] * m for i in range(n)]
        self.player_list.append(Player(1, 0, "White"))
        self.player_list.append(Player(2, 0, "Black"))
        print(self.player_list[0].nb, self.player_list[1].nb)

    def check_win_rust(self, player, x, y):
        return(gomoku_rust.check_win(self.table,player,x,y))

    def place_stone(self, player, x, y):
        Rust_res = gomoku_rust.place_stone(self.table, player.nb, x, y)
        print(Rust_res)
        if Rust_res["game_status"] == -1:
            return -1
        else:
            self.table = Rust_res["board"]
            player.eat_piece += Rust_res["eated_piece"]
            if player.eat_piece >= 10:
                return player.nb
            if "wining_position" in Rust_res.keys():
                for p in self.player_list:
                    if p.nb == player.nb:
                        p.wining_position.append([x, y])
            return 0

    def print_game_status(self):
        for player in self.player_list:
            print("player: ", player.color, "")

    def reset_players(self):
        self.player_list.clear()
        self.player_list.append(Player(1, 0, "White"))
        self.player_list.append(Player(2, 0, "Black"))
        print("ici printing player list")
        print(self.player_list)

    def reset_game(self):
        self.reset_players()
        self.clear_board()

    def clear_board(self):
        for L in range(len(self.table)):
            for l in range((len(self.table[L]))):
                self.table[L][l] = 0
        self.print_table()