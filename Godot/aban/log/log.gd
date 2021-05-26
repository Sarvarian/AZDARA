extends Node


func error(msg : String) -> void:
	var datetime : String = $Rust.get_current_datetime()
	var fullmsg = datetime + " " + msg
	$Rust.log_without_time(fullmsg)
	$Console.log_put(datetime, msg, Color.red)
	printerr(fullmsg)
