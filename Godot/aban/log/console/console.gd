extends CanvasLayer


func _ready() -> void:
	put("Welcome to AZDARA", Color.white)


func put(msg : String, color : Color) -> void:
	$Control/Wall.append_bbcode("\n")
	$Control/Wall.push_color(color)
	$Control/Wall.append_bbcode(msg)
	$Control/Wall.pop()


func alert(msg : String, color : Color) -> void:
	$Alert.alert(msg, color)


func log_put(datetime : String, msg : String, color : Color) -> void:
	alert(msg, color)
	$Control/Wall.append_bbcode("\n")
	$Control/Wall.push_color(Color.green)
	$Control/Wall.append_bbcode(datetime)
	$Control/Wall.pop()
	$Control/Wall.append_bbcode(" ")
	$Control/Wall.push_color(color)
	$Control/Wall.append_bbcode(msg)
	$Control/Wall.pop()


func _on_Check_AZDARA_Lib_azdara_doesnt_find(name : String) -> void:
	var msg : String = name + " doesn't find!"
	alert(msg , Color.red)
	put(msg, Color.red)
