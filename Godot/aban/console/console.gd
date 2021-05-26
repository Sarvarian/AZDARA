extends CanvasLayer


func _ready() -> void:
	put_white("Welcome to AZDARA")


var time : float = 10
var count : int = 0


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


func red_alert(text : String) -> void:
	put_red(text)
	$Alert.alert(text, Color.red)


func yellow_alert(text : String) -> void:
	put_yellow(text)
	$Alert.alert(text, Color.yellow)


func white_alert(text : String) -> void:
	put_white(text)
	$Alert.alert(text, Color.white)
