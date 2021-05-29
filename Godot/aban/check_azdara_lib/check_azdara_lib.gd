extends Node


onready var dialog : AcceptDialog = $"../AcceptDialog"


func _ready() -> void:
	dialog.get_close_button().hide()
	dialog.dialog_text = azdara_file_name() + " doesn't find!"
	
	print("--- Start checking for AZDARA lib")
	
	var exe := check_exe_path_for_azdara()
	var res := check_res_for_azdara()
	
	if not exe and not res:
		dialog.popup_centered()
		printerr("Founding AZDARA Lib Failed")
		push_error("Founding AZDARA Lib Failed")
	else:
		get_parent().queue_free()
	
	print("--- End checking for AZDARA lib")


func _on_AcceptDialog_confirmed():
	get_tree().quit(-1)


func check_res_for_azdara() -> bool:
	var directory := Directory.new()
	var err := directory.open("res://")
	if err:
		push_error("'res://' failed to open. Godot Error Code: " + String(err))
	
	return directory.file_exists("aban/lib/" + azdara_file_name())


func check_exe_path_for_azdara() -> bool:
	var exe_path : String = OS.get_executable_path().get_base_dir()
	var directory := Directory.new()
	var err := directory.open(exe_path)
	if err:
		push_error(exe_path + " failed to open. Godot Error Code: " + String(err))
	
	return directory.file_exists(azdara_file_name())


func azdara_file_name() -> String:
	var name : String = "azdara"
	
	match OS.get_name():
		"Windows":
			name = name + ".dll"
	
	return name











