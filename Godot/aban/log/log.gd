extends Node


func _ready() -> void:
	var check : bool = $Rust.check()
	if not check:
		$Console/AZDARA_PopError.popup_centered()
		printerr("Founding AZDARA Lib Failed")
		push_error("Founding AZDARA Lib Failed")
		$Console.AZDARA_PopError("azdara lib")
	else:
		$Console/AZDARA_PopError.queue_free()
		$Console.remove_child($Console/AZDARA_PopError)


func error(msg : String) -> void:
	var datetime : String = $Rust.get_current_datetime()
	var fullmsg = datetime + " " + msg
	$Rust.log_without_time(fullmsg)
	$Console.log_put(datetime, msg, Color.red)
	printerr(fullmsg)
