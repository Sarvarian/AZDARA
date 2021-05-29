extends Panel


const norm_style : StyleBoxFlat = preload("res://aban/game_select/new_game_card/assets/norm_style.tres")
const hover_style : StyleBoxFlat = preload("res://aban/game_select/new_game_card/assets/hover_style.tres")


onready var label : Label = $Label


func _ready() -> void:
	var err : int = 0 # TODO Handle Errors
	connect("mouse_entered", self, "_on_Panel_mouse_entered", [], CONNECT_DEFERRED)
	connect("mouse_exited", self, "_on_Panel_mouse_exited", [], CONNECT_DEFERRED)
	connect("focus_entered", self, "_on_Panel_focus_entered", [], CONNECT_DEFERRED)
	connect("focus_exited", self, "_on_Panel_focus_exited", [], CONNECT_DEFERRED)
	connect("gui_input", self, "_on_Panel_gui_input", [], CONNECT_DEFERRED)


func _on_Panel_mouse_entered() -> void:
	grab_focus()


func _on_Panel_mouse_exited() -> void:
	release_focus()


func _on_Panel_focus_entered() -> void:
	set("custom_styles/panel", hover_style)
	label.set("custom_colors/font_color", Color.red)


func _on_Panel_focus_exited() -> void:
	set("custom_styles/panel", norm_style)
	label.set("custom_colors/font_color", Color.white)


func _on_Panel_gui_input(event : InputEvent) -> void:
	if event.is_action_released("ui_accept"):
		$CreateGame.pop()
	elif event is InputEventMouseButton:
		if event.button_index == 1:
			if event.is_pressed():
				$CreateGame.pop()
