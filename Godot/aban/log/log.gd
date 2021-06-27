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
		$Rust.welcome_message("Welcome to AZDARA!", Color(1.0, 0.1, 0.6))


func error(msg : String) -> void:
	$Rust.error(msg)
