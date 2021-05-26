class_name Game
extends Resource


var game := preload("res://aban/lib/game.gdns").new()


func _init(name : String) -> void:
	game._init(name)


func _exit_tree() -> void:
	game.free()
