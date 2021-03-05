class_name Game
extends Node


onready var lib :=  preload("res://aban/lib/game.gdns").new()


func _ready() -> void:
	lib.ready("myGame")
	lib.save()
	pass


func _process(delta : float) -> void:
	lib.process(delta)


func _physics_process(delta : float) -> void:
	lib.physics_process(delta)


func _exit_tree() -> void:
	lib.free()


func set_viewport_world(viewport : Viewport) -> void:
	viewport.world = lib.get_world()
