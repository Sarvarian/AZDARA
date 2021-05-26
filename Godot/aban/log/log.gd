extends Node


func error(text : String) -> void:
	var msg := _format_log_get("Error: " + text)
	printerr(msg)
	push_error(msg)
	Console.put_red(msg)


func info(text : String) -> void:
	var msg := _format_log_get("Info: " + text)
	print(msg)
	Console.put_white(msg)


func warning(text : String) -> void:
	var msg := _format_log_get("Warning: " + text)
	push_warning(msg)
	Console.put_yellow(msg)


func _format_log_get(text : String) -> String:
	var datetime : String = $Rust.get_current_datetime()
	var msg : String = "{} {}".format([
		datetime,
		text
	], "{}")
	
	$Rust.log_without_time(msg)
	return msg
