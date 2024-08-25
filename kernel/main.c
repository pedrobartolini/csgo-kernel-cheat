#pragma warning (disable : 4100 4047 4024 4820 4311 4022 4201)

#include <ntifs.h>
#include <ntddk.h>
#include <minwindef.h>

#include "types.h"

//#define Print(x, ...) DbgPrintEx(0, 0, x, __VA_ARGS__);

#include "hide.h"
#include "process.h"
#include "ioctl.h"


NTSTATUS UnloadDriver(PDRIVER_OBJECT pDriverObject)
{
	// DEINIT IOCTL
	if (pDriverObject->DeviceObject != NULL) {
		IoDeleteSymbolicLink(&dos_device_unicode);
		IoDeleteDevice(pDriverObject->DeviceObject);
	}

	// DEINIT CS HOOKER
	PsSetCreateProcessNotifyRoutineEx(HookProcess, TRUE);
	PsRemoveLoadImageNotifyRoutine(HookModules);

	return STATUS_SUCCESS;
}

NTSTATUS DriverEntry(PDRIVER_OBJECT pDriverObject, PUNICODE_STRING pRegistryPath)
{
	pDriverObject->DriverUnload = UnloadDriver;

	RtlInitUnicodeString(&device_unicode, L"\\Device\\magicplatearmor");
	RtlInitUnicodeString(&dos_device_unicode, L"\\DosDevices\\magicplatearmor");
	IoCreateDevice(pDriverObject, 0, &device_unicode, FILE_DEVICE_UNKNOWN, FILE_DEVICE_SECURE_OPEN, FALSE, &device_object);
	IoCreateSymbolicLink(&dos_device_unicode, &device_unicode);
	pDriverObject->MajorFunction[IRP_MJ_CREATE] = IoctlOpen;
	pDriverObject->MajorFunction[IRP_MJ_CLOSE] = IoctlClose;
	pDriverObject->MajorFunction[IRP_MJ_DEVICE_CONTROL] = IoctlControl;
	device_object->Flags |= DO_DIRECT_IO;
	device_object->Flags &= ~DO_DEVICE_INITIALIZING;

	PsSetCreateProcessNotifyRoutineEx(HookProcess, FALSE);
	PsSetLoadImageNotifyRoutine(HookModules);

	PLDR_DATA_TABLE_ENTRY CurDriverEntry = (PLDR_DATA_TABLE_ENTRY)pDriverObject->DriverSection;
	PLDR_DATA_TABLE_ENTRY NextDriverEntry = (PLDR_DATA_TABLE_ENTRY)CurDriverEntry->InLoadOrderLinks.Flink;
	PLDR_DATA_TABLE_ENTRY PrevDriverEntry = (PLDR_DATA_TABLE_ENTRY)CurDriverEntry->InLoadOrderLinks.Blink;

	PrevDriverEntry->InLoadOrderLinks.Flink = CurDriverEntry->InLoadOrderLinks.Flink;
	NextDriverEntry->InLoadOrderLinks.Blink = CurDriverEntry->InLoadOrderLinks.Blink;

	CurDriverEntry->InLoadOrderLinks.Flink = (PLIST_ENTRY)CurDriverEntry;
	CurDriverEntry->InLoadOrderLinks.Blink = (PLIST_ENTRY)CurDriverEntry;

	
	return STATUS_SUCCESS;
}











