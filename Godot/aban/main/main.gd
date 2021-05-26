extends Node


func _input(event : InputEvent) -> void:
	if event.is_action_released("fullscreen"):
		if OS.is_window_fullscreen():
			OS.set_window_fullscreen(false)
		else:
			OS.set_window_fullscreen(true)
