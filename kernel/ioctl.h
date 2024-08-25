#define READ CTL_CODE(FILE_DEVICE_UNKNOWN, 0x1, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define WRITE CTL_CODE(FILE_DEVICE_UNKNOWN, 0x2, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)

#define GET_CLIENT CTL_CODE(FILE_DEVICE_UNKNOWN, 0x10, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define GET_ENGINE CTL_CODE(FILE_DEVICE_UNKNOWN, 0x11, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)

#define PING CTL_CODE(FILE_DEVICE_UNKNOWN, 0x20, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define INTERNAL_READ CTL_CODE(FILE_DEVICE_UNKNOWN, 0x21, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)


UNICODE_STRING device_unicode;
UNICODE_STRING dos_device_unicode;
PDEVICE_OBJECT device_object;

NTSTATUS NTAPI MmCopyVirtualMemory(
	PEPROCESS src_process,
	PVOID src_addr,
	PEPROCESS target_process,
	PVOID target_addr,
	SIZE_T size,
	KPROCESSOR_MODE mode,
	PSIZE_T return_size
);

typedef struct _MMCOPY {
	ULONG addr;
	PVOID buff;
	ULONG size;
} MMCOPY, * PMMCOPY;

NTSTATUS CopyMemory(PEPROCESS src_process, PVOID src_addr, PEPROCESS target_process, PVOID target_addr, SIZE_T size)
{
	PSIZE_T bytes;
	return MmCopyVirtualMemory(src_process, src_addr, target_process, target_addr, size, KernelMode, &bytes);
}

NTSTATUS IoctlControl(PDEVICE_OBJECT DeviceObject, PIRP Irp) {
	NTSTATUS status = STATUS_SUCCESS;

	PIO_STACK_LOCATION irp_slot = IoGetCurrentIrpStackLocation(Irp);
	ULONG msg_code = irp_slot->Parameters.DeviceIoControl.IoControlCode;
	ULONG msg_size = 0;

	if (msg_code == READ && CSGO) {
		PMMCOPY mmcopy = (PMMCOPY)Irp->AssociatedIrp.SystemBuffer;
		CopyMemory(CSGO, mmcopy->addr, PsGetCurrentProcess(), mmcopy->buff, mmcopy->size);
		msg_size = sizeof(MMCOPY);
	}

	else if (msg_code == WRITE && CSGO) {
		PMMCOPY mmcopy = (PMMCOPY)Irp->AssociatedIrp.SystemBuffer;
		CopyMemory(PsGetCurrentProcess(), mmcopy->buff, CSGO, mmcopy->addr, mmcopy->size);
		msg_size = sizeof(MMCOPY);
	}

	else if (msg_code == GET_CLIENT && CSGO && client) {
		*(PULONG)Irp->AssociatedIrp.SystemBuffer = client;
		msg_size = sizeof(client);
	}
	
	else if (msg_code == GET_ENGINE && CSGO && engine) {
		*(PULONG)Irp->AssociatedIrp.SystemBuffer = engine;
		msg_size = sizeof(engine);
	}

	else if (msg_code == PING) {
		*(UINT8*)Irp->AssociatedIrp.SystemBuffer = 1;
		msg_size = sizeof(UINT8);
	}

	else {
		status = STATUS_UNSUCCESSFUL;
	}

	Irp->IoStatus.Status = status;
	Irp->IoStatus.Information = msg_size;
	IoCompleteRequest(Irp, IO_NO_INCREMENT);

	return status;
}

NTSTATUS IoctlOpen(PDEVICE_OBJECT DeviceObject, PIRP Irp) {
	Irp->IoStatus.Status = STATUS_SUCCESS;
	Irp->IoStatus.Information = 0;
	IoCompleteRequest(Irp, IO_NO_INCREMENT);
	return STATUS_SUCCESS;
}

NTSTATUS IoctlClose(PDEVICE_OBJECT DeviceObject, PIRP Irp) {
	Irp->IoStatus.Status = STATUS_SUCCESS;
	Irp->IoStatus.Information = 0;
	IoCompleteRequest(Irp, IO_NO_INCREMENT);
	return STATUS_SUCCESS;
}

