/* should not generate diagnostics */
({ 0: "zero" });
({ 1: "one" });
({ 1.2: "12" });
({ 3.1e12: "12" });
({ 0.1e12: "ee" });
({
	// n
	20: "20"
});
({ a });
({ ...a });
