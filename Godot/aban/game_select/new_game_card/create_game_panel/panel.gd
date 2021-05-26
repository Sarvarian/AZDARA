extends Panel


const res_packages_directory : String = "res://packages/"
const user_packages_directory : String = "user://packages/"


var packages : PoolStringArray = []
var directory : Directory = Directory.new()


onready var err_label : Label = $"../../Control/ErrLabel"


func _ready() -> void:
	var err : int = 0
	
	if not directory.dir_exists(user_packages_directory):
		err = directory.make_dir_recursive(user_packages_directory)
	check_and_report_err(err)
	
	check_dir_for_packages(res_packages_directory)
	check_dir_for_packages(user_packages_directory)


func check_dir_for_packages(dir : String) -> void:
	var err : int = 0
	
	err = directory.open(dir)
	check_and_report_err(err)
	
	err = directory.list_dir_begin(true, false)
	check_and_report_err(err)
	
	var name := directory.get_next()
	while name != "":
		add_to_packages(name)
		name = directory.get_next()
	
	directory.list_dir_end()
	


func check_and_report_err(err : int) -> void:
	if not err:
		return
	err_label.text += "Godot Error code: {}\n".format([err], "{}")


func add_to_packages(name : String) -> void:
	if directory.current_is_dir():
		packages.push_back(name)




