class_name Game
extends Reference


var game := preload("res://aban/lib/game.gdns").new()


func _init(name : String) -> void:
	game._init(name)


func _notification(what : int) -> void:
	if what == NOTIFICATION_PREDELETE:
		game.free()
