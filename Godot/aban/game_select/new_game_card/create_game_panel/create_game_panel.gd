extends CanvasLayer


func _ready() -> void:
	$Control.hide()
	$Margin.hide()


func pop() -> void:
	$Control.show()
	$Margin.show()


func _on_Control_focus_entered() -> void:
	$Margin.hide()
	$Control.hide()
