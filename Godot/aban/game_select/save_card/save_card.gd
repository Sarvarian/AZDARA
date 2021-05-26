extends Control



onready var panel : Panel = $Margin/AspectRatio/Panel


func set_name(save_name : String) -> void:
	name = save_name
	$Margin/AspectRatio/Panel/Name.text = name


func _on_SaveCard_focus_entered() -> void:
	panel.grab_focus()


func _on_Panel_mouse_entered() -> void:
	panel.grab_focus()


func _on_Panel_mouse_exited() -> void:
	panel.release_focus()



