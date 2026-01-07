import { styled } from "@mui/material/styles";
import { useState, useRef, useEffect } from "react";
import { useTranslation } from "react-i18next";
import formulaCategories, { FormulaCategory, FormulaInfo } from "./FormulaCategories";

interface FunctionLibraryProps {
    onInsertFormula: (formula: string) => void;
    canEdit: boolean;
}

function FunctionLibrary({ onInsertFormula, canEdit }: FunctionLibraryProps) {
    const { t } = useTranslation();
    const [openCategory, setOpenCategory] = useState<string | null>(null);
    const [searchQuery, setSearchQuery] = useState("");
    const dropdownRef = useRef<HTMLDivElement>(null);

    // Close dropdown when clicking outside
    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
                setOpenCategory(null);
            }
        };
        document.addEventListener("mousedown", handleClickOutside);
        return () => document.removeEventListener("mousedown", handleClickOutside);
    }, []);

    const handleCategoryClick = (categoryId: string) => {
        setOpenCategory(openCategory === categoryId ? null : categoryId);
        setSearchQuery("");
    };

    const handleFormulaClick = (formula: FormulaInfo) => {
        if (!canEdit) return;
        // Insert formula with parentheses ready for input
        onInsertFormula(`=${formula.name}()`);
        setOpenCategory(null);
    };

    const filterFormulas = (formulas: FormulaInfo[]) => {
        if (!searchQuery) return formulas;
        const query = searchQuery.toLowerCase();
        return formulas.filter(
            f => f.name.toLowerCase().includes(query) ||
                f.localName.toLowerCase().includes(query) ||
                f.description.toLowerCase().includes(query)
        );
    };

    // Get all formulas for search
    const allFormulas = formulaCategories.flatMap(cat => cat.formulas);
    const searchResults = searchQuery ? filterFormulas(allFormulas) : [];

    return (
        <LibraryWrapper ref={dropdownRef}>
            {/* Search box */}
            <SearchBox>
                <SearchInput
                    type="text"
                    placeholder={t("ribbon.search_function", "🔍 Поиск функции...")}
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    onFocus={() => setOpenCategory("search")}
                />
                {searchQuery && openCategory === "search" && searchResults.length > 0 && (
                    <SearchDropdown>
                        {searchResults.slice(0, 10).map((formula) => (
                            <FormulaItem
                                key={formula.name}
                                onClick={() => handleFormulaClick(formula)}
                                disabled={!canEdit}
                            >
                                <FormulaName>{formula.localName}</FormulaName>
                                <FormulaDesc>{formula.description}</FormulaDesc>
                            </FormulaItem>
                        ))}
                    </SearchDropdown>
                )}
            </SearchBox>

            {/* Category buttons */}
            <CategoryList>
                {formulaCategories.map((category) => (
                    <CategoryButton
                        key={category.id}
                        $active={openCategory === category.id}
                        onClick={() => handleCategoryClick(category.id)}
                        title={category.name}
                    >
                        <CategoryIcon>{category.icon}</CategoryIcon>
                        <CategoryName>{category.name}</CategoryName>
                    </CategoryButton>
                ))}
            </CategoryList>

            {/* Category dropdown */}
            {openCategory && openCategory !== "search" && (
                <CategoryDropdown>
                    {formulaCategories
                        .filter((cat) => cat.id === openCategory)
                        .map((category) => (
                            <DropdownContent key={category.id}>
                                <DropdownHeader>
                                    <span>{category.icon}</span>
                                    <span>{category.name}</span>
                                </DropdownHeader>
                                <FormulaList>
                                    {filterFormulas(category.formulas).map((formula) => (
                                        <FormulaItem
                                            key={formula.name}
                                            onClick={() => handleFormulaClick(formula)}
                                            disabled={!canEdit}
                                        >
                                            <FormulaName>{formula.localName}</FormulaName>
                                            <FormulaDesc>{formula.description}</FormulaDesc>
                                            <FormulaSyntax>{formula.syntax}</FormulaSyntax>
                                        </FormulaItem>
                                    ))}
                                </FormulaList>
                            </DropdownContent>
                        ))}
                </CategoryDropdown>
            )}
        </LibraryWrapper>
    );
}

// Styled components
const LibraryWrapper = styled("div")({
    display: "flex",
    alignItems: "center",
    gap: "8px",
    position: "relative",
    flex: 1,
});

const SearchBox = styled("div")({
    position: "relative",
});

const SearchInput = styled("input")(({ theme }) => ({
    width: "160px",
    padding: "6px 8px",
    border: `1px solid ${theme.palette.divider}`,
    borderRadius: "4px",
    fontSize: "11px",
    backgroundColor: theme.palette.background.default,
    color: theme.palette.text.primary,
    "&:focus": {
        outline: "none",
        borderColor: theme.palette.primary.main,
    },
    "&::placeholder": {
        color: theme.palette.text.secondary,
    },
}));

const SearchDropdown = styled("div")(({ theme }) => ({
    position: "absolute",
    top: "100%",
    left: 0,
    right: 0,
    marginTop: "4px",
    backgroundColor: theme.palette.background.paper,
    border: `1px solid ${theme.palette.divider}`,
    borderRadius: "4px",
    boxShadow: "0 4px 12px rgba(0,0,0,0.15)",
    zIndex: 1000,
    maxHeight: "300px",
    overflowY: "auto",
}));

const CategoryList = styled("div")({
    display: "flex",
    gap: "4px",
    flexWrap: "nowrap",
    overflowX: "auto",
    "&::-webkit-scrollbar": {
        display: "none",
    },
});

interface CategoryButtonProps {
    $active: boolean;
}

const CategoryButton = styled("button")<CategoryButtonProps>(({ theme, $active }) => ({
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    padding: "4px 8px",
    border: "none",
    backgroundColor: $active ? theme.palette.primary.main + "20" : "transparent",
    color: $active ? theme.palette.primary.main : theme.palette.text.primary,
    cursor: "pointer",
    borderRadius: "4px",
    fontSize: "9px",
    minWidth: "56px",
    transition: "all 0.15s ease",
    "&:hover": {
        backgroundColor: theme.palette.action.hover,
    },
}));

const CategoryIcon = styled("span")({
    fontSize: "18px",
    marginBottom: "2px",
});

const CategoryName = styled("span")({
    whiteSpace: "nowrap",
    overflow: "hidden",
    textOverflow: "ellipsis",
    maxWidth: "60px",
});

const CategoryDropdown = styled("div")(({ theme }) => ({
    position: "absolute",
    top: "100%",
    left: 0,
    marginTop: "4px",
    backgroundColor: theme.palette.background.paper,
    border: `1px solid ${theme.palette.divider}`,
    borderRadius: "8px",
    boxShadow: "0 8px 24px rgba(0,0,0,0.2)",
    zIndex: 1000,
    minWidth: "320px",
    maxHeight: "400px",
    overflowY: "auto",
}));

const DropdownContent = styled("div")({
    padding: "8px 0",
});

const DropdownHeader = styled("div")(({ theme }) => ({
    display: "flex",
    alignItems: "center",
    gap: "8px",
    padding: "8px 12px",
    fontSize: "13px",
    fontWeight: 600,
    color: theme.palette.text.primary,
    borderBottom: `1px solid ${theme.palette.divider}`,
    marginBottom: "4px",
}));

const FormulaList = styled("div")({
    display: "flex",
    flexDirection: "column",
});

const FormulaItem = styled("button")(({ theme }) => ({
    display: "flex",
    flexDirection: "column",
    alignItems: "flex-start",
    padding: "8px 12px",
    border: "none",
    backgroundColor: "transparent",
    cursor: "pointer",
    textAlign: "left",
    width: "100%",
    transition: "all 0.1s ease",
    "&:hover:not(:disabled)": {
        backgroundColor: theme.palette.action.hover,
    },
    "&:disabled": {
        opacity: 0.5,
        cursor: "not-allowed",
    },
}));

const FormulaName = styled("span")(({ theme }) => ({
    fontSize: "12px",
    fontWeight: 600,
    color: theme.palette.primary.main,
}));

const FormulaDesc = styled("span")(({ theme }) => ({
    fontSize: "11px",
    color: theme.palette.text.secondary,
    marginTop: "2px",
}));

const FormulaSyntax = styled("span")(({ theme }) => ({
    fontSize: "10px",
    color: theme.palette.text.disabled,
    fontFamily: "monospace",
    marginTop: "4px",
}));

export default FunctionLibrary;
