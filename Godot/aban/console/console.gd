extends CanvasLayer


func _ready() -> void:
	put_white("Welcome to AZDARA")


func put_white(text : String) -> void:
	put(text, Color.white)


func put_red(text : String) -> void:
	put(text, Color.red)


func put_yellow(text : String) -> void:
	put(text, Color.yellow)


func put(text : String, color : Color) -> void:
	$Control/Wall.append_bbcode("\n")
	$Control/Wall.push_color(color)
	$Control/Wall.append_bbcode(text)
	$Control/Wall.pop()


func alert(_text : String) -> void:
	pass

