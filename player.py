class Player:
	nb = 0
	player_type = 0
	eat_piece = 0
	color = None

	def __init__(self,nb,player_type,color):
		self.nb = nb
		self.player_type = player_type
		self.color = color
