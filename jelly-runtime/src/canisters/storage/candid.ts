export const idlFactory = ({ IDL }: any) => {
    const CanisterStatusType = IDL.Variant({
        stopped: IDL.Null,
        stopping: IDL.Null,
        running: IDL.Null,
    });
    const LogVisibility = IDL.Variant({
        controllers: IDL.Null,
        public: IDL.Null,
    });
    const DefiniteCanisterSettings = IDL.Record({
        freezing_threshold: IDL.Nat,
        controllers: IDL.Vec(IDL.Principal),
        reserved_cycles_limit: IDL.Nat,
        log_visibility: LogVisibility,
        wasm_memory_limit: IDL.Nat,
        memory_allocation: IDL.Nat,
        compute_allocation: IDL.Nat,
    });
    const QueryStats = IDL.Record({
        response_payload_bytes_total: IDL.Nat,
        num_instructions_total: IDL.Nat,
        num_calls_total: IDL.Nat,
        request_payload_bytes_total: IDL.Nat,
    });
    const CanisterStatusResponse = IDL.Record({
        status: CanisterStatusType,
        memory_size: IDL.Nat,
        cycles: IDL.Nat,
        settings: DefiniteCanisterSettings,
        query_stats: QueryStats,
        idle_cycles_burned_per_day: IDL.Nat,
        module_hash: IDL.Opt(IDL.Vec(IDL.Nat8)),
        reserved_cycles: IDL.Nat,
    });
    const Result = IDL.Variant({ Ok: IDL.Text, Err: IDL.Text });
    return IDL.Service({
        __get_candid_interface_tmp_hack: IDL.Func([], [IDL.Text], ['query']),
        admin_add: IDL.Func([IDL.Principal], [], []),
        admin_query: IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
        admin_remove: IDL.Func([IDL.Principal], [], []),
        api_query: IDL.Func([IDL.Text], [IDL.Opt(IDL.Text)], ['query']),
        api_update: IDL.Func([IDL.Text], [], []),
        canister_status: IDL.Func([], [CanisterStatusResponse], []),
        code_query: IDL.Func([IDL.Text], [IDL.Opt(IDL.Text)], ['query']),
        code_update: IDL.Func([IDL.Text], [], []),
        combined_increment_called: IDL.Func([IDL.Text], [], []),
        combined_query: IDL.Func([IDL.Text], [IDL.Opt(IDL.Text)], ['query']),
        combined_update: IDL.Func([IDL.Text], [], []),
        dapp_increment_called_by_admin: IDL.Func([IDL.Text], [], []),
        dapp_increment_called_by_token: IDL.Func([IDL.Text, IDL.Opt(IDL.Text)], [], []),
        dapp_query_access: IDL.Func([IDL.Text], [Result], ['query']),
        dapp_query_by_admin: IDL.Func([IDL.Text], [Result], ['query']),
        dapp_query_by_token: IDL.Func([IDL.Text, IDL.Opt(IDL.Text)], [Result], ['query']),
        dapp_update: IDL.Func([IDL.Text], [], []),
        dapp_update_collected: IDL.Func([IDL.Text, IDL.Nat64], [], []),
        publisher_query: IDL.Func([IDL.Text], [IDL.Opt(IDL.Text)], ['query']),
        publisher_update: IDL.Func([IDL.Text], [], []),
        wallet_balance: IDL.Func([], [IDL.Nat], ['query']),
        whoami: IDL.Func([], [IDL.Principal], ['query']),
    });
};
