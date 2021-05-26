extends Camera2D


onready var character := $"../Character"


func _process(_delta : float) -> void:
	position = (character.visual_rep as Node2D).position
