
PEPROCESS CSGO;
HANDLE CSGO_PID;
INT32 client, engine;

PLOAD_IMAGE_NOTIFY_ROUTINE HookModules(PUNICODE_STRING FullImageName, HANDLE ProcessId, PIMAGE_INFO ImageInfo) {
	if (wcsstr(FullImageName->Buffer, L"\\Counter-Strike Global Offensive\\csgo\\bin\\client.dll")) {
		client = ImageInfo->ImageBase;
	}

	if (wcsstr(FullImageName->Buffer, L"\\Counter-Strike Global Offensive\\bin\\engine.dll")) {
		engine = ImageInfo->ImageBase;
	}

	return STATUS_SUCCESS;
}


PCREATE_PROCESS_NOTIFY_ROUTINE_EX HookProcess(HANDLE ParentId, HANDLE ProcessId, PPS_CREATE_NOTIFY_INFO Creating) {
	if (Creating) {
		if (wcsstr(Creating->ImageFileName->Buffer, L"\\csgo.exe")) {
			CSGO_PID = ProcessId;
			PsLookupProcessByProcessId(ProcessId, &CSGO);
		}

		else if (wcsstr(Creating->ImageFileName->Buffer, L"\\crst.exe"))
		{
			HideProcess((UINT32)ProcessId);
		}
	}


	else if (ProcessId == CSGO_PID) {
		CSGO_PID = NULL;
		CSGO = NULL;
	}

	return STATUS_SUCCESS;
}


