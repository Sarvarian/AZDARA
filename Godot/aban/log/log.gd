extends Node


func _ready() -> void:
	error("Hello!")
	warning("Hello!")
	info("Hello!")


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
	var datetime := OS.get_datetime()
	var datetime_text := "[{}-{}-{} {}:{}:{}]".format(
		[
			datetime.year,
			datetime.month,
			datetime.day,
			datetime.hour,
			datetime.minute,
			datetime.second,
		], "{}")
	
	
	var final_text : String = "{} {}".format([
		datetime_text,
		text
	], "{}")
	
	$Rust.log_without_time(final_text)
	return final_text
