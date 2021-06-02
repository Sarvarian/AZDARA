extends Panel


const norm_style : StyleBoxFlat = preload("res://aban/game_select/new_game_card/assets/norm_style.tres")
const hover_style : StyleBoxFlat = preload("res://aban/game_select/new_game_card/assets/hover_style.tres")
const siganl_callback : Array = [
	["mouse_entered", "_on_Panel_mouse_entered"],
	["mouse_exited", "_on_Panel_mouse_exited"],
	["focus_entered", "_on_Panel_focus_entered"],
	["focus_exited", "_on_Panel_focus_exited"],
	["gui_input", "_on_Panel_gui_input"]
]


func _ready() -> void:
	for s in siganl_callback:
		var name : String = s[0]
		var callback : String = s[1]
		var err : int = connect(name, self, callback, [], CONNECT_DEFERRED)
		if err:
			var msg = signal_connection_err_msg(name, err)
			Log.error(msg)
			push_error(msg)


func _on_Panel_mouse_entered() -> void:
	grab_focus()


func _on_Panel_mouse_exited() -> void:
	release_focus()


func _on_Panel_focus_entered() -> void:
	set("custom_styles/panel", hover_style)
	$Label.set("custom_colors/font_color", Color.red)


func _on_Panel_focus_exited() -> void:
	set("custom_styles/panel", norm_style)
	$Label.set("custom_colors/font_color", Color.white)


func _on_Panel_gui_input(event : InputEvent) -> void:
	if event.is_action_released("ui_accept"):
		$CreateGame.pop()
	elif event is InputEventMouseButton:
		if event.button_index == 1:
			if event.is_pressed():
				print("TODO: pop create story game panel")


func signal_connection_err_msg(signal_name : String, error_code : int) -> String:
	return "New Game Card Panel failed to connect " + signal_name + " signal. Godot Error Code : " + String(error_code)
