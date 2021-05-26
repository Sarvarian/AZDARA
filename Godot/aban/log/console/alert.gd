extends Control


const alert_scene := preload("res://aban/log/console/alert/alert.tscn")


func alert(text : String, color : Color) -> void:
	var alert_msg : Label = alert_scene.instance()
	alert_msg.text = text
	alert_msg.modulate = color
	$VBox.add_child(alert_msg)
