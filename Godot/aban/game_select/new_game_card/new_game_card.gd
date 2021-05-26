extends Control


onready var panel : Panel = $Margin/AspectRatio/Panel


func _on_Panel_mouse_entered() -> void:
	panel.grab_focus()


func _on_Panel_mouse_exited() -> void:
	panel.release_focus()
