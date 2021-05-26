extends Panel


const res_packages_directory : String = "res://packages/"
const user_packages_directory : String = "user://packages/"


var user_packages : PoolStringArray = []
var res_packages : PoolStringArray = []
var directory : Directory = Directory.new()


onready var err_label : Label = $"../../Control/ErrLabel"


func _ready() -> void:
	var err : int = 0
	
	if not directory.dir_exists(user_packages_directory):
		err = directory.make_dir_recursive(user_packages_directory)
	check_and_report_err(err)
	
	res_packages = check_dir_for_packages(res_packages_directory)
	user_packages = check_dir_for_packages(user_packages_directory)
	
	$Tree.set_packages(res_packages)
	$Tree.set_packages(user_packages)


func check_dir_for_packages(dir : String) -> PoolStringArray:
	var err : int = 0
	var packages : PoolStringArray = []
	
	err = directory.open(dir)
	check_and_report_err(err)
	
	err = directory.list_dir_begin(true, false)
	check_and_report_err(err)
	
	var name := directory.get_next()
	while name != "":
		if directory.current_is_dir():
			packages.push_back(name)
		name = directory.get_next()
	
	directory.list_dir_end()
	
	return packages


func check_and_report_err(err : int) -> void:
	if not err:
		return
	err_label.text += "Godot Error code: {}\n".format([err], "{}")





