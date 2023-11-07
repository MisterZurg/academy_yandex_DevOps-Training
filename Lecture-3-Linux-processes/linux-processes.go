package main

import (
	"fmt"
	"github.com/shirou/gopsutil/disk"
	"github.com/shirou/gopsutil/mem"
	"os"
	"runtime"
)

// GetProcessInfo решает шаг 01 — собрать информацию о текущем процессе
func GetProcessInfo() {
	//processID := os.Getpid()
	//fmt.Println("Process Id is", processID)

	// Получение и вывод количества дескрипторов
	fdCount, err := getDescriptorCount()
	if err == nil {
		fmt.Printf("Количество дескрипторов: %d\n", fdCount)
	} else {
		fmt.Println("Ошибка при получении количества дескрипторов")
	}

	// Получение и вывод потребления памяти
	memUsage := runtime.MemStats{}
	runtime.ReadMemStats(&memUsage)
	fmt.Printf("Потребление памяти: %d bytes\n", memUsage.Alloc)

	// Получение и вывод пути до исполняемого файла в проекте i donno
	//pwd, err := os.Getwd()
	//if err != nil {
	//	fmt.Println(err)
	//	os.Exit(1)
	//}
	//fmt.Println(pwd)

	// Получение и вывод пути до исполняемого файла
	executablePath, err := os.Executable()
	if err == nil {
		fmt.Printf("Путь до исполняемого файла: %s\n", executablePath)
	} else {
		fmt.Println("Ошибка при получении пути до исполняемого файла")
	}
}

// getDescriptorCount хелпер для получения количества дескрипторов
func getDescriptorCount() (int, error) {
	var OS_PATH string
	operatingSystem := runtime.GOOS

	switch operatingSystem {
	case "darwin":
		// macOS использует дескрипторы файлов и сокеты
		fmt.Println("MAC operating system")
		OS_PATH = "/dev"
	case "linux":
		fmt.Println("Linux")
		OS_PATH = "/proc/self/fd"
	default:
		fmt.Println("f u windows")
		panic("production ready")
	}

	file, err := os.Open(OS_PATH)
	if err != nil {
		return 0, err
	}
	defer file.Close()

	files, err := file.Readdirnames(0)
	if err != nil {
		return 0, err
	}

	return len(files), nil
}

// GetSystemInfo решает шаг 02 — собрать информацию о системе
func GetSystemInfo() {
	// Информация о процессоре:
	fmt.Println(runtime.GOOS)     // darwin aka "MAC operating system"
	fmt.Println(runtime.NumCPU()) // Количество процессоров

	// Информация о памяти:
	virtualMemory, err := mem.VirtualMemory()
	if err == nil {
		fmt.Printf("Всего памяти: %v GB\n", virtualMemory.Total/1024/1024/1024)
		fmt.Printf("Свободно памяти: %v GB\n", virtualMemory.Free/1024/1024/1024)
	} else {
		fmt.Println("Ошибка при получении информации о памяти")
	}

	// Информация о дисках:
	partitions, err := disk.Partitions(false)
	if err == nil {
		for _, partition := range partitions {
			fmt.Printf("Устройство: %s\n", partition.Device)
			fmt.Printf("Тип: %s\n", partition.Fstype)
		}
	} else {
		fmt.Println("Ошибка при получении информации о дисках")
	}
}

func main() {
	// TODO : Гигалень собирать JSON
	GetProcessInfo()
	GetSystemInfo()
}
