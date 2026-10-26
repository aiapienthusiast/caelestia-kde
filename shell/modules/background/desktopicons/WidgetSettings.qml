pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Caelestia.Config
import qs.components
import qs.components.controls
import qs.components.effects
import qs.services

// "Widget Settings": what the card looks like and, from the catalogue's schema, the
// widget's own controls. The form is rebuilt on every opening so its fields always
// start on the values the widget really has.
Item {
    id: root

    property var controller: null
    property bool open: false
    property point at
    property string widgetId: ""

    readonly property var widget: widgetId !== "" ? (DesktopLayout.widgets[widgetId] ?? null) : null
    readonly property string type: widget?.type ?? ""
    readonly property var info: type !== "" ? (controller?.widgetCatalog?.info(type) ?? null) : null
    readonly property string title: info?.name ?? qsTr("Widget")
    // The form reports how tall it wants to be, so a two-field widget does not get a
    // screen-tall panel and a long one still fits.
    readonly property real formHeight: form.item ? form.item.implicitHeight : 0
    readonly property real maxFormHeight: Math.max(120, root.height * 0.55)

    function openAt(x: real, y: real, id: string): void {
        at = Qt.point(x, y);
        widgetId = id;
        open = true;
        // A second opening for another widget must not show the fields the previous
        // one left behind: destroy the form and build it again.
        form.active = false;
        Qt.callLater(() => {
            form.active = root.open && root.type !== "";
        });
    }

    function close(): void {
        open = false;
        form.active = false;
    }

    anchors.fill: parent
    visible: panel.opacity > 0

    onWidgetChanged: {
        // The widget was removed while its settings were open; there is nothing left
        // to configure, so the panel goes with it.
        if (open && widget === null)
            close();
    }

    // Leaving the editor leaves the panel behind it, or it would float over the
    // desktop with nothing to edit.
    Connections {
        function onActiveChanged(): void {
            if (!WidgetEditor.active)
                root.close();
        }

        target: WidgetEditor
    }

    MouseArea {
        anchors.fill: parent
        enabled: root.open
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        onPressed: root.close()
    }

    Elevation {
        id: panel

        x: Math.max(Tokens.padding.large, Math.min(root.width - width - Tokens.padding.large, root.at.x))
        y: Math.max(Tokens.padding.large, Math.min(root.height - height - Tokens.padding.large, root.at.y))
        width: Math.min(340, root.width - Tokens.padding.large * 2)
        height: body.implicitHeight + Tokens.padding.large * 2
        radius: Tokens.rounding.large
        level: 2
        opacity: root.open ? 1 : 0
        scale: root.open ? 1 : 0.9
        transformOrigin: Item.TopLeft

        Behavior on opacity {
            Anim {
                type: Anim.FastEffects
            }
        }

        Behavior on scale {
            Anim {
                type: Anim.FastSpatial
            }
        }

        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.LeftButton | Qt.RightButton
        }

        StyledRect {
            anchors.fill: parent
            radius: parent.radius
            color: GlobalConfig.appearance.pitchBlack ? "#000000" : Colours.palette.m3surfaceContainerLow

            ColumnLayout {
                id: body

                anchors.fill: parent
                anchors.margins: Tokens.padding.large
                spacing: Tokens.spacing.medium

                RowLayout {
                    Layout.fillWidth: true
                    spacing: Tokens.spacing.small

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 0

                        StyledText {
                            Layout.fillWidth: true
                            text: root.title
                            font: Tokens.font.title.small
                            elide: Text.ElideRight
                        }

                        StyledText {
                            Layout.fillWidth: true
                            text: qsTr("Edit the widget's settings")
                            color: Colours.palette.m3onSurfaceVariant
                            font: Tokens.font.label.small
                            elide: Text.ElideRight
                        }
                    }

                    TextButton {
                        text: qsTr("Reset")
                        type: TextButton.Text
                        onClicked: form.item?.reset()
                    }
                }

                Flickable {
                    id: scroll

                    Layout.fillWidth: true
                    Layout.preferredHeight: Math.min(root.formHeight, root.maxFormHeight)
                    contentWidth: width
                    contentHeight: form.height
                    clip: true
                    boundsBehavior: Flickable.StopAtBounds

                    ScrollBar.vertical: StyledScrollBar {
                        flickable: scroll
                    }

                    Loader {
                        id: form

                        width: scroll.width
                        active: false
                        sourceComponent: WidgetSettingsForm {
                            widgetId: root.widgetId
                            type: root.type
                            catalog: root.controller?.widgetCatalog ?? null
                        }
                    }
                }
            }
        }
    }
}
