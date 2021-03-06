class_name Game
extends Node


const GAME_NAME : String = "MyGame"

onready var lib :=  preload("res://aban/lib/game.gdns").new()


func _ready() -> void:
	lib.ready()
	pass


func _process(delta : float) -> void:
	lib.process(delta)


func _physics_process(delta : float) -> void:
	lib.physics_process(delta)


func _exit_tree() -> void:
	lib.free()


func _unhandled_input(event : InputEvent) -> void:
	lib.input(event)
	if event.is_action_pressed("save_game"):
		lib.save_game(GAME_NAME)
	if event.is_action_pressed("load_game"):
		lib.load_game(GAME_NAME)
	pass


func set_viewport_world(viewport : Viewport) -> void:
	viewport.world = lib.get_world()
