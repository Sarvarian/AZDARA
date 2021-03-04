class_name Game
extends Node


onready var lib : Object =  preload("res://aban/lib/game.gdns").new()


func _exit_tree() -> void:
	lib.free()


func set_viewport_world(viewport : Viewport) -> void:
	viewport.world = lib.get_world()
