extends Panel


const norm_style : StyleBoxFlat = preload("res://aban/game_select/new_game_card/norm_style.tres")
const hover_style : StyleBoxFlat = preload("res://aban/game_select/new_game_card/hover_style.tres")


onready var label : Label = $Label


func _on_Panel_focus_entered() -> void:
	set("custom_styles/panel", hover_style)
	label.set("custom_colors/font_color", Color.red)


func _on_Panel_focus_exited() -> void:
	set("custom_styles/panel", norm_style)
	label.set("custom_colors/font_color", Color.white)
