class_name PlayerViewport
extends ViewportContainer


func set_world_2d(world_2d : World2D) -> void:
	$Viewport.set_world_2d(world_2d)


func set_camera_target(target : Node2D) -> void:
	$Viewport/PlayerCamera.target = target

