extends Control


var viewports : Dictionary = {}

onready var player_viewport_scene : PackedScene = load(Addresses.PLAYER_VIEWPORT_SCENE)
onready var grid : GridContainer = $Panel/MarginContainer/GridContainer


func add_player_camera(target : Node2D, uuid : String, world_2d : World2D) -> void:
	var player_viewport : PlayerViewport = player_viewport_scene.instance()
	player_viewport.set_world_2d(world_2d)
	player_viewport.set_camera_target(target)
	grid.add_child(player_viewport)
	viewports[uuid] = player_viewport
	check_grid_child_count()


func remove_viewport(uuid : String) -> void:
	grid.remove_child(viewports[uuid])
	(viewports[uuid] as Node).queue_free()
# warning-ignore:return_value_discarded
	viewports.erase(uuid)


func clear_viewports() -> void:
	for v in grid.get_children():
		grid.remove_child(v)
		(v as Node).queue_free()
	viewports.clear()


func check_grid_child_count() -> void:
	var count : int = grid.get_child_count()
	if count < 2:
		grid.columns = 1
	elif count < 5:
		grid.columns = 2
	elif count < 10:
		grid.columns = 3
	else:
		grid.columns = 4


