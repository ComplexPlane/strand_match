//@author ComplexPlane
//@category SMB
//@keybinding
//@menupath
//@toolbar

import ghidra.app.script.GhidraScript;
import ghidra.app.util.exporter.XmlExporter;
import ghidra.program.model.listing.*;
import ghidra.program.model.symbol.*;
import ghidra.program.model.address.*;
import java.io.File;
import java.io.BufferedReader;
import java.io.FileReader;

public class StrandMatchImport extends GhidraScript {

	@Override
	protected void run() throws Exception {
		SymbolTable symbolTable = currentProgram.getSymbolTable();

		BufferedReader mapFile = new BufferedReader(new FileReader(askFile("Select a symbol map to import.\nNote that this will overwrite existing symbols at the given addresses.", "Import")));
		String line;

		while ((line = mapFile.readLine()) != null) {
			String[] split = line.split(" ");
			String newSymbol = split[0];
			Address addr = toAddr(split[1]);

			Symbol[] userSymbols = symbolTable.getUserSymbols(addr);
			for (Symbol s : userSymbols) {
				symbolTable.removeSymbolSpecial(s);
			}

			createLabel(addr, newSymbol, true);
		}

		// removeSymbolSpecial() supposedly will remove symbol references, so re-analyze to restore them
		analyzeAll(currentProgram);
	}
}
