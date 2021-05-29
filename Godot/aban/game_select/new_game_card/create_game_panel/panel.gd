extends Panel


const res_packages_directory : String = "rses://packages/"
const user_packages_directory : String = "usser://packages/"


var user_packages : PoolStringArray = []
var res_packages : PoolStringArray = []
var directory : Directory = Directory.new()


func _ready() -> void:
	var err : int = 0
	
	if not directory.dir_exists(user_packages_directory):
		err = directory.make_dir_recursive(user_packages_directory)
		if err:
			Log.error("Creating User Packages Directory Failed With Godot Error Code: " + String(err))
	
	res_packages = check_dir_for_packages(res_packages_directory)
	user_packages = check_dir_for_packages(user_packages_directory)
	
	$Tree.set_packages(res_packages)
	$Tree.set_packages(user_packages)


func check_dir_for_packages(dir : String) -> PoolStringArray:
	var err : int = 0
	var packages : PoolStringArray = []
	
	err = directory.open(dir)
	if err:
		Log.error("Opennig Directory '" + dir + "' Failed With Godot Error Code: " + String(err))
	
	err = directory.list_dir_begin(true, false)
	if err:
		Log.error("'" + dir + "'.list_dir_begin(true, false) Failed With Godot Error Code: " + String(err))
	
	var name := directory.get_next()
	while name != "":
		if directory.current_is_dir():
			packages.push_back(name)
		name = directory.get_next()
	
	directory.list_dir_end()
	
	return packages





