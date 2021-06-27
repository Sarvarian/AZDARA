extends Node


func _ready() -> void:
	var check : bool = $Rust.check()
	if not check:
		$Rust/AZDARA_PopError.popup_centered()
		printerr("Founding AZDARA Lib Failed")
		push_error("Founding AZDARA Lib Failed")
		$Rust.AZDARA_PopError("azdara lib")
	else:
		$Rust/AZDARA_PopError.queue_free()
		$Rust.remove_child($Rust/AZDARA_PopError)


func error(msg : String) -> void:
	$Rust.error(msg)
#	var datetime : String = $Rust.get_current_datetime()
#	var fullmsg = datetime + " " + msg
#	$Rust.log_without_time(fullmsg)
#	$Console.log_put(datetime, msg, Color.red)
#	printerr(fullmsg)
