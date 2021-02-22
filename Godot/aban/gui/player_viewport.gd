class_name PlayerViewport
extends MarginContainer


func set_camera_target(target : Node2D) -> void:
	$ViewportContainer/Viewport/PlayerCamera.target = target
