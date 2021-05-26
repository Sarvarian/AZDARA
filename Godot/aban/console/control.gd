extends Control


func _ready() -> void:
	visible = false


func _input(event : InputEvent) -> void:
	if visible:
		if event.is_action_released("console") or event.is_action_released("ui_cancel"):
			close()
	else:
		if event.is_action_released("console"):
			open()


func _on_Close_pressed():
	close()


func open() -> void:
	visible = true
	$Input.grab_focus()

func close() -> void:
	visible = false
	release_focus()

