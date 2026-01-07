import { styled } from "@mui/material/styles";
import { useState, useRef } from "react";
import { useTranslation } from "react-i18next";
import FunctionLibrary from "./FunctionLibrary";

export const RIBBON_HEIGHT = 100;

interface RibbonProps {
    onInsertFormula: (formula: string) => void;
    canEdit: boolean;
}

type TabId = "home" | "insert" | "formulas" | "data" | "view";

interface TabConfig {
    id: TabId;
    labelKey: string;
}

const tabs: TabConfig[] = [
    { id: "home", labelKey: "ribbon.tabs.home" },
    { id: "insert", labelKey: "ribbon.tabs.insert" },
    { id: "formulas", labelKey: "ribbon.tabs.formulas" },
    { id: "data", labelKey: "ribbon.tabs.data" },
    { id: "view", labelKey: "ribbon.tabs.view" },
];

function Ribbon({ onInsertFormula, canEdit }: RibbonProps) {
    const { t } = useTranslation();
    const [activeTab, setActiveTab] = useState<TabId>("home");
    const ribbonRef = useRef<HTMLDivElement>(null);

    const renderTabContent = () => {
        switch (activeTab) {
            case "home":
                return (
                    <TabContent>
                        <Group>
                            <GroupLabel>{t("ribbon.groups.clipboard", "Буфер обмена")}</GroupLabel>
                            <GroupContent>
                                <ActionButton disabled={!canEdit}>
                                    <span>📋</span>
                                    <span>{t("ribbon.actions.paste", "Вставить")}</span>
                                </ActionButton>
                                <SmallButtonGroup>
                                    <SmallButton disabled={!canEdit} title={t("ribbon.actions.cut", "Вырезать")}>✂️</SmallButton>
                                    <SmallButton disabled={!canEdit} title={t("ribbon.actions.copy", "Копировать")}>📄</SmallButton>
                                </SmallButtonGroup>
                            </GroupContent>
                        </Group>
                        <Divider />
                        <Group>
                            <GroupLabel>{t("ribbon.groups.font", "Шрифт")}</GroupLabel>
                            <GroupContent>
                                <SmallButtonGroup>
                                    <SmallButton disabled={!canEdit}><strong>Ж</strong></SmallButton>
                                    <SmallButton disabled={!canEdit}><em>К</em></SmallButton>
                                    <SmallButton disabled={!canEdit}><u>Ч</u></SmallButton>
                                </SmallButtonGroup>
                            </GroupContent>
                        </Group>
                        <Divider />
                        <Group>
                            <GroupLabel>{t("ribbon.groups.alignment", "Выравнивание")}</GroupLabel>
                            <GroupContent>
                                <SmallButtonGroup>
                                    <SmallButton disabled={!canEdit}>⬅️</SmallButton>
                                    <SmallButton disabled={!canEdit}>↔️</SmallButton>
                                    <SmallButton disabled={!canEdit}>➡️</SmallButton>
                                </SmallButtonGroup>
                            </GroupContent>
                        </Group>
                    </TabContent>
                );

            case "formulas":
                return (
                    <TabContent>
                        <Group style={{ flex: 1 }}>
                            <GroupLabel>{t("ribbon.groups.function_library", "Библиотека функций")}</GroupLabel>
                            <GroupContent>
                                <FunctionLibrary
                                    onInsertFormula={onInsertFormula}
                                    canEdit={canEdit}
                                />
                            </GroupContent>
                        </Group>
                    </TabContent>
                );

            case "insert":
                return (
                    <TabContent>
                        <Group>
                            <GroupLabel>{t("ribbon.groups.charts", "Диаграммы")}</GroupLabel>
                            <GroupContent>
                                <ActionButton disabled>
                                    <span>📊</span>
                                    <span>{t("ribbon.actions.chart", "Диаграмма")}</span>
                                </ActionButton>
                            </GroupContent>
                        </Group>
                        <Divider />
                        <Group>
                            <GroupLabel>{t("ribbon.groups.links", "Ссылки")}</GroupLabel>
                            <GroupContent>
                                <ActionButton disabled>
                                    <span>🔗</span>
                                    <span>{t("ribbon.actions.link", "Ссылка")}</span>
                                </ActionButton>
                            </GroupContent>
                        </Group>
                    </TabContent>
                );

            case "data":
                return (
                    <TabContent>
                        <Group>
                            <GroupLabel>{t("ribbon.groups.sort_filter", "Сортировка и фильтр")}</GroupLabel>
                            <GroupContent>
                                <ActionButton disabled>
                                    <span>🔽</span>
                                    <span>{t("ribbon.actions.sort_asc", "А-Я")}</span>
                                </ActionButton>
                                <ActionButton disabled>
                                    <span>🔼</span>
                                    <span>{t("ribbon.actions.sort_desc", "Я-А")}</span>
                                </ActionButton>
                            </GroupContent>
                        </Group>
                    </TabContent>
                );

            case "view":
                return (
                    <TabContent>
                        <Group>
                            <GroupLabel>{t("ribbon.groups.show", "Показать")}</GroupLabel>
                            <GroupContent>
                                <ActionButton disabled={!canEdit}>
                                    <span>📐</span>
                                    <span>{t("ribbon.actions.gridlines", "Сетка")}</span>
                                </ActionButton>
                                <ActionButton disabled={!canEdit}>
                                    <span>🔢</span>
                                    <span>{t("ribbon.actions.headers", "Заголовки")}</span>
                                </ActionButton>
                            </GroupContent>
                        </Group>
                    </TabContent>
                );

            default:
                return null;
        }
    };

    return (
        <RibbonWrapper ref={ribbonRef}>
            <TabBar>
                {tabs.map((tab) => (
                    <Tab
                        key={tab.id}
                        $active={activeTab === tab.id}
                        onClick={() => setActiveTab(tab.id)}
                    >
                        {t(tab.labelKey, tab.id)}
                    </Tab>
                ))}
            </TabBar>
            <RibbonContent>
                {renderTabContent()}
            </RibbonContent>
        </RibbonWrapper>
    );
}

// Styled components
const RibbonWrapper = styled("div")(({ theme }) => ({
    height: `${RIBBON_HEIGHT}px`,
    backgroundColor: theme.palette.background.paper,
    borderBottom: `1px solid ${theme.palette.divider}`,
    display: "flex",
    flexDirection: "column",
    fontFamily: theme.typography.fontFamily,
}));

const TabBar = styled("div")(({ theme }) => ({
    display: "flex",
    gap: "2px",
    padding: "4px 8px 0",
    backgroundColor: theme.palette.mode === "dark" ? "#1e1e2e" : "#f0f0f5",
}));

interface TabProps {
    $active: boolean;
}

const Tab = styled("button")<TabProps>(({ theme, $active }) => ({
    padding: "6px 16px",
    border: "none",
    backgroundColor: $active
        ? theme.palette.background.paper
        : "transparent",
    color: $active
        ? theme.palette.primary.main
        : theme.palette.text.secondary,
    cursor: "pointer",
    fontSize: "12px",
    fontWeight: $active ? 600 : 400,
    borderRadius: "4px 4px 0 0",
    transition: "all 0.15s ease",
    "&:hover": {
        backgroundColor: $active
            ? theme.palette.background.paper
            : theme.palette.action.hover,
    },
}));

const RibbonContent = styled("div")(({ theme }) => ({
    flex: 1,
    display: "flex",
    alignItems: "stretch",
    padding: "4px 8px",
    overflowX: "auto",
    backgroundColor: theme.palette.background.paper,
    "&::-webkit-scrollbar": {
        height: "4px",
    },
    "&::-webkit-scrollbar-thumb": {
        backgroundColor: theme.palette.divider,
        borderRadius: "2px",
    },
}));

const TabContent = styled("div")({
    display: "flex",
    alignItems: "stretch",
    gap: "4px",
    flex: 1,
});

const Group = styled("div")({
    display: "flex",
    flexDirection: "column",
    padding: "4px 8px",
    minWidth: "80px",
});

const GroupLabel = styled("div")(({ theme }) => ({
    fontSize: "10px",
    color: theme.palette.text.secondary,
    textAlign: "center",
    marginTop: "auto",
    paddingTop: "4px",
    borderTop: `1px solid ${theme.palette.divider}`,
}));

const GroupContent = styled("div")({
    display: "flex",
    alignItems: "center",
    gap: "4px",
    flex: 1,
});

const Divider = styled("div")(({ theme }) => ({
    width: "1px",
    backgroundColor: theme.palette.divider,
    margin: "4px 8px",
}));

const ActionButton = styled("button")(({ theme }) => ({
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    padding: "4px 8px",
    border: "none",
    backgroundColor: "transparent",
    cursor: "pointer",
    borderRadius: "4px",
    fontSize: "10px",
    color: theme.palette.text.primary,
    transition: "all 0.15s ease",
    minWidth: "50px",
    "& > span:first-of-type": {
        fontSize: "20px",
        marginBottom: "2px",
    },
    "&:hover:not(:disabled)": {
        backgroundColor: theme.palette.action.hover,
    },
    "&:disabled": {
        opacity: 0.5,
        cursor: "not-allowed",
    },
}));

const SmallButtonGroup = styled("div")({
    display: "flex",
    gap: "2px",
});

const SmallButton = styled("button")(({ theme }) => ({
    width: "24px",
    height: "24px",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    border: "none",
    backgroundColor: "transparent",
    cursor: "pointer",
    borderRadius: "4px",
    fontSize: "12px",
    "&:hover:not(:disabled)": {
        backgroundColor: theme.palette.action.hover,
    },
    "&:disabled": {
        opacity: 0.5,
        cursor: "not-allowed",
    },
}));

export default Ribbon;
